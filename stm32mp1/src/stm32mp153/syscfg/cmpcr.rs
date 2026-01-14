///Register `CMPCR` reader
pub type R = crate::R<CMPCRrs>;
///Register `CMPCR` writer
pub type W = crate::W<CMPCRrs>;
///Field `SW_CTRL` reader - SW_CTRL
pub type SW_CTRL_R = crate::BitReader;
///Field `SW_CTRL` writer - SW_CTRL
pub type SW_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READY` reader - READY
pub type READY_R = crate::BitReader;
///Field `RANSRC` reader - RANSRC
pub type RANSRC_R = crate::FieldReader;
///Field `RANSRC` writer - RANSRC
pub type RANSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RAPSRC` reader - RAPSRC
pub type RAPSRC_R = crate::FieldReader;
///Field `RAPSRC` writer - RAPSRC
pub type RAPSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ANSRC` reader - ANSRC
pub type ANSRC_R = crate::FieldReader;
///Field `APSRC` reader - APSRC
pub type APSRC_R = crate::FieldReader;
impl R {
    ///Bit 1 - SW_CTRL
    #[inline(always)]
    pub fn sw_ctrl(&self) -> SW_CTRL_R {
        SW_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - READY
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:19 - RANSRC
    #[inline(always)]
    pub fn ransrc(&self) -> RANSRC_R {
        RANSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - RAPSRC
    #[inline(always)]
    pub fn rapsrc(&self) -> RAPSRC_R {
        RAPSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - ANSRC
    #[inline(always)]
    pub fn ansrc(&self) -> ANSRC_R {
        ANSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - APSRC
    #[inline(always)]
    pub fn apsrc(&self) -> APSRC_R {
        APSRC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPCR")
            .field("sw_ctrl", &self.sw_ctrl())
            .field("ready", &self.ready())
            .field("ransrc", &self.ransrc())
            .field("rapsrc", &self.rapsrc())
            .field("ansrc", &self.ansrc())
            .field("apsrc", &self.apsrc())
            .finish()
    }
}
impl W {
    ///Bit 1 - SW_CTRL
    #[inline(always)]
    pub fn sw_ctrl(&mut self) -> SW_CTRL_W<'_, CMPCRrs> {
        SW_CTRL_W::new(self, 1)
    }
    ///Bits 16:19 - RANSRC
    #[inline(always)]
    pub fn ransrc(&mut self) -> RANSRC_W<'_, CMPCRrs> {
        RANSRC_W::new(self, 16)
    }
    ///Bits 20:23 - RAPSRC
    #[inline(always)]
    pub fn rapsrc(&mut self) -> RAPSRC_W<'_, CMPCRrs> {
        RAPSRC_W::new(self, 20)
    }
}
/**SYSCFG compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`cmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:CMPCR)*/
pub struct CMPCRrs;
impl crate::RegisterSpec for CMPCRrs {
    type Ux = u32;
}
///`read()` method returns [`cmpcr::R`](R) reader structure
impl crate::Readable for CMPCRrs {}
///`write(|w| ..)` method takes [`cmpcr::W`](W) writer structure
impl crate::Writable for CMPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMPCR to value 0x0087_0000
impl crate::Resettable for CMPCRrs {
    const RESET_VALUE: u32 = 0x0087_0000;
}
