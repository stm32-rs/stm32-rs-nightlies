///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
///Field `TRIM_CC1_RD` reader - SW trim value for Rd resistor on the CC1 line
pub type TRIM_CC1_RD_R = crate::FieldReader;
///Field `TRIM_CC1_RD` writer - SW trim value for Rd resistor on the CC1 line
pub type TRIM_CC1_RD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIM_CC1_RP` reader - SW trim value for Rp current sources on the CC1 line
pub type TRIM_CC1_RP_R = crate::FieldReader;
///Field `TRIM_CC1_RP` writer - SW trim value for Rp current sources on the CC1 line
pub type TRIM_CC1_RP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIM_CC2_RD` reader - SW trim value for Rd resistor on the CC2 line
pub type TRIM_CC2_RD_R = crate::FieldReader;
///Field `TRIM_CC2_RD` writer - SW trim value for Rd resistor on the CC2 line
pub type TRIM_CC2_RD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIM_CC2_RP` reader - SW trim value for Rp current sources on the CC2 line
pub type TRIM_CC2_RP_R = crate::FieldReader;
///Field `TRIM_CC2_RP` writer - SW trim value for Rp current sources on the CC2 line
pub type TRIM_CC2_RP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - SW trim value for Rd resistor on the CC1 line
    #[inline(always)]
    pub fn trim_cc1_rd(&self) -> TRIM_CC1_RD_R {
        TRIM_CC1_RD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 9:12 - SW trim value for Rp current sources on the CC1 line
    #[inline(always)]
    pub fn trim_cc1_rp(&self) -> TRIM_CC1_RP_R {
        TRIM_CC1_RP_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 16:19 - SW trim value for Rd resistor on the CC2 line
    #[inline(always)]
    pub fn trim_cc2_rd(&self) -> TRIM_CC2_RD_R {
        TRIM_CC2_RD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 25:28 - SW trim value for Rp current sources on the CC2 line
    #[inline(always)]
    pub fn trim_cc2_rp(&self) -> TRIM_CC2_RP_R {
        TRIM_CC2_RP_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("trim_cc1_rd", &self.trim_cc1_rd())
            .field("trim_cc1_rp", &self.trim_cc1_rp())
            .field("trim_cc2_rd", &self.trim_cc2_rd())
            .field("trim_cc2_rp", &self.trim_cc2_rp())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SW trim value for Rd resistor on the CC1 line
    #[inline(always)]
    pub fn trim_cc1_rd(&mut self) -> TRIM_CC1_RD_W<'_, CFGR3rs> {
        TRIM_CC1_RD_W::new(self, 0)
    }
    ///Bits 9:12 - SW trim value for Rp current sources on the CC1 line
    #[inline(always)]
    pub fn trim_cc1_rp(&mut self) -> TRIM_CC1_RP_W<'_, CFGR3rs> {
        TRIM_CC1_RP_W::new(self, 9)
    }
    ///Bits 16:19 - SW trim value for Rd resistor on the CC2 line
    #[inline(always)]
    pub fn trim_cc2_rd(&mut self) -> TRIM_CC2_RD_W<'_, CFGR3rs> {
        TRIM_CC2_RD_W::new(self, 16)
    }
    ///Bits 25:28 - SW trim value for Rp current sources on the CC2 line
    #[inline(always)]
    pub fn trim_cc2_rp(&mut self) -> TRIM_CC2_RP_W<'_, CFGR3rs> {
        TRIM_CC2_RP_W::new(self, 25)
    }
}
/**UCPD configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#UCPD1:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {}
