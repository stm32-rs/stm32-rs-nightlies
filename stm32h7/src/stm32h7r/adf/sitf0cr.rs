///Register `SITF0CR` reader
pub type R = crate::R<SITF0CRrs>;
///Register `SITF0CR` writer
pub type W = crate::W<SITF0CRrs>;
///Field `SITFEN` reader - Serial interface enable This bit is set and cleared by software. It is used to enable/disable the serial interface.
pub type SITFEN_R = crate::BitReader;
///Field `SITFEN` writer - Serial interface enable This bit is set and cleared by software. It is used to enable/disable the serial interface.
pub type SITFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCKSRC` reader - Serial clock source This field is set and cleared by software. It is used to select the clock source of the serial interface. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type SCKSRC_R = crate::FieldReader;
///Field `SCKSRC` writer - Serial clock source This field is set and cleared by software. It is used to select the clock source of the serial interface. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type SCKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SITFMOD` reader - Serial interface type This field is set and cleared by software. It is used to define the serial interface type. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type SITFMOD_R = crate::FieldReader;
///Field `SITFMOD` writer - Serial interface type This field is set and cleared by software. It is used to define the serial interface type. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type SITFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STH` reader - Manchester symbol threshold/SPI threshold This field is set and cleared by software. It is used for Manchester mode to define the expected symbol threshold levels (seer to Manchester mode for details on computation). In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. STH\[4:0\] values lower than four are invalid. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type STH_R = crate::FieldReader;
///Field `STH` writer - Manchester symbol threshold/SPI threshold This field is set and cleared by software. It is used for Manchester mode to define the expected symbol threshold levels (seer to Manchester mode for details on computation). In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. STH\[4:0\] values lower than four are invalid. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type STH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SITFACTIVE` reader - Serial interface active flag This bit is set and cleared by hardware. It is used by the application to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when SITFACTIVE is set to 0 (see Section 46.4.13: Register protection for details). The delay between a transition on SITFEN and a transition on SITFACTIVE is two periods of AHB clock and two periods of adf_proc_ck.
pub type SITFACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - Serial interface enable This bit is set and cleared by software. It is used to enable/disable the serial interface.
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Serial clock source This field is set and cleared by software. It is used to select the clock source of the serial interface. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - Serial interface type This field is set and cleared by software. It is used to define the serial interface type. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:12 - Manchester symbol threshold/SPI threshold This field is set and cleared by software. It is used for Manchester mode to define the expected symbol threshold levels (seer to Manchester mode for details on computation). In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. STH\[4:0\] values lower than four are invalid. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 31 - Serial interface active flag This bit is set and cleared by hardware. It is used by the application to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when SITFACTIVE is set to 0 (see Section 46.4.13: Register protection for details). The delay between a transition on SITFEN and a transition on SITFACTIVE is two periods of AHB clock and two periods of adf_proc_ck.
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SITF0CR")
            .field("sitfen", &self.sitfen())
            .field("scksrc", &self.scksrc())
            .field("sitfmod", &self.sitfmod())
            .field("sth", &self.sth())
            .field("sitfactive", &self.sitfactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Serial interface enable This bit is set and cleared by software. It is used to enable/disable the serial interface.
    #[inline(always)]
    pub fn sitfen(&mut self) -> SITFEN_W<'_, SITF0CRrs> {
        SITFEN_W::new(self, 0)
    }
    ///Bits 1:2 - Serial clock source This field is set and cleared by software. It is used to select the clock source of the serial interface. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn scksrc(&mut self) -> SCKSRC_W<'_, SITF0CRrs> {
        SCKSRC_W::new(self, 1)
    }
    ///Bits 4:5 - Serial interface type This field is set and cleared by software. It is used to define the serial interface type. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn sitfmod(&mut self) -> SITFMOD_W<'_, SITF0CRrs> {
        SITFMOD_W::new(self, 4)
    }
    ///Bits 8:12 - Manchester symbol threshold/SPI threshold This field is set and cleared by software. It is used for Manchester mode to define the expected symbol threshold levels (seer to Manchester mode for details on computation). In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. STH\[4:0\] values lower than four are invalid. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn sth(&mut self) -> STH_W<'_, SITF0CRrs> {
        STH_W::new(self, 8)
    }
}
/**ADF serial interface control register 0

You can [`read`](crate::Reg::read) this register and get [`sitf0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ADF:SITF0CR)*/
pub struct SITF0CRrs;
impl crate::RegisterSpec for SITF0CRrs {
    type Ux = u32;
}
///`read()` method returns [`sitf0cr::R`](R) reader structure
impl crate::Readable for SITF0CRrs {}
///`write(|w| ..)` method takes [`sitf0cr::W`](W) writer structure
impl crate::Writable for SITF0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SITF0CR to value 0x1f00
impl crate::Resettable for SITF0CRrs {
    const RESET_VALUE: u32 = 0x1f00;
}
