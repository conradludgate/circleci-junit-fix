<?xml version="1.0"?>

<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:template match="/testsuites">
        <testsuites>
            <xsl:attribute name="name">
                <xsl:value-of select="@name" />
            </xsl:attribute>
            <xsl:attribute name="tests">
                <xsl:value-of select="@tests" />
            </xsl:attribute>
            <xsl:attribute name="failures">
                <xsl:value-of select="@failures" />
            </xsl:attribute>
            <xsl:attribute name="errors">
                <xsl:value-of select="@errors" />
            </xsl:attribute>
            <xsl:attribute name="timestamp">
                <xsl:value-of select="@timestamp" />
            </xsl:attribute>
            <xsl:attribute name="time">
                <xsl:value-of select="@time" />
            </xsl:attribute>

            <xsl:for-each select="testsuite">
                <testsuite>
                    <xsl:attribute name="name">
                        <xsl:value-of select="@name" />
                    </xsl:attribute>
                    <xsl:attribute name="tests">
                        <xsl:value-of select="@tests" />
                    </xsl:attribute>
                    <xsl:attribute name="disabled">
                        <xsl:value-of select="@disabled" />
                    </xsl:attribute>
                    <xsl:attribute name="errors">
                        <xsl:value-of select="@errors" />
                    </xsl:attribute>
                    <xsl:attribute name="failures">
                        <xsl:value-of select="@failures" />
                    </xsl:attribute>

                    <xsl:for-each select="testcase">
                        <testcase>
                            <xsl:attribute name="name">
                                <xsl:value-of select="@name" />
                            </xsl:attribute>
                            <xsl:attribute name="classname">
                                <xsl:value-of select="@classname" />
                            </xsl:attribute>
                            <xsl:attribute name="timestamp">
                                <xsl:value-of select="@timestamp" />
                            </xsl:attribute>
                            <xsl:attribute name="time">
                                <xsl:value-of select="@time" />
                            </xsl:attribute>

                            <xsl:for-each select="failure">
                                <failure>
                                    <xsl:attribute name="type">
                                        <xsl:value-of select="@type" />
                                    </xsl:attribute>
                                    <xsl:attribute name="message">--- STDERR:
<xsl:value-of select="../system-err" /></xsl:attribute>
--- STDOUT:
<xsl:value-of select="../system-out" /></failure>
                            </xsl:for-each>
                        </testcase>
                    </xsl:for-each>
                </testsuite>
            </xsl:for-each>
        </testsuites>
    </xsl:template>
</xsl:stylesheet>
