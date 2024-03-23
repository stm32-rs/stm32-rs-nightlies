#[doc = "Register `MDF_SITF2CR` reader"]
pub type R = crate::R<MDF_SITF2CRrs>;
#[doc = "Register `MDF_SITF2CR` writer"]
pub type W = crate::W<MDF_SITF2CRrs>;
#[doc = "Field `SITFEN` reader - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled"]
pub type SITFEN_R = crate::BitReader;
#[doc = "Field `SITFEN` writer - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled"]
pub type SITFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCKSRC` reader - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type SCKSRC_R = crate::FieldReader;
#[doc = "Field `SCKSRC` writer - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type SCKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SITFMOD` reader - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type SITFMOD_R = crate::FieldReader;
#[doc = "Field `SITFMOD` writer - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type SITFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STH` reader - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\\[4:0\\]
lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type STH_R = crate::FieldReader;
#[doc = "Field `STH` writer - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\\[4:0\\]
lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type STH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SITFACTIVE` reader - Serial interface Active flag"]
pub type SITFACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled"]
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\\[4:0\\]
lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Serial interface Active flag"]
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sitfen(&mut self) -> SITFEN_W<MDF_SITF2CRrs> {
        SITFEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn scksrc(&mut self) -> SCKSRC_W<MDF_SITF2CRrs> {
        SCKSRC_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn sitfmod(&mut self) -> SITFMOD_W<MDF_SITF2CRrs> {
        SITFMOD_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\\[4:0\\]
lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn sth(&mut self) -> STH_W<MDF_SITF2CRrs> {
        STH_W::new(self, 8)
    }
}
#[doc = "This register is used to control the serial interfaces (SITFx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_sitf2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdf_sitf2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_SITF2CRrs;
impl crate::RegisterSpec for MDF_SITF2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_sitf2cr::R`](R) reader structure"]
impl crate::Readable for MDF_SITF2CRrs {}
#[doc = "`write(|w| ..)` method takes [`mdf_sitf2cr::W`](W) writer structure"]
impl crate::Writable for MDF_SITF2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDF_SITF2CR to value 0x1f00"]
impl crate::Resettable for MDF_SITF2CRrs {
    const RESET_VALUE: u32 = 0x1f00;
}
