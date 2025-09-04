///Register `MP_APB3LPENCLRR` reader
pub type R = crate::R<MP_APB3LPENCLRRrs>;
///Register `MP_APB3LPENCLRR` writer
pub type W = crate::W<MP_APB3LPENCLRRrs>;
///Field `LPTIM2LPEN` reader - LPTIM2LPEN
pub type LPTIM2LPEN_R = crate::BitReader;
///Field `LPTIM2LPEN` writer - LPTIM2LPEN
pub type LPTIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3LPEN` reader - LPTIM3LPEN
pub type LPTIM3LPEN_R = crate::BitReader;
///Field `LPTIM3LPEN` writer - LPTIM3LPEN
pub type LPTIM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4LPEN` reader - LPTIM4LPEN
pub type LPTIM4LPEN_R = crate::BitReader;
///Field `LPTIM4LPEN` writer - LPTIM4LPEN
pub type LPTIM4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5LPEN` reader - LPTIM5LPEN
pub type LPTIM5LPEN_R = crate::BitReader;
///Field `LPTIM5LPEN` writer - LPTIM5LPEN
pub type LPTIM5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI4LPEN` reader - SAI4LPEN
pub type SAI4LPEN_R = crate::BitReader;
///Field `SAI4LPEN` writer - SAI4LPEN
pub type SAI4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGLPEN` reader - SYSCFGLPEN
pub type SYSCFGLPEN_R = crate::BitReader;
///Field `SYSCFGLPEN` writer - SYSCFGLPEN
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFLPEN` reader - VREFLPEN
pub type VREFLPEN_R = crate::BitReader;
///Field `VREFLPEN` writer - VREFLPEN
pub type VREFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSLPEN` reader - DTSLPEN
pub type DTSLPEN_R = crate::BitReader;
///Field `DTSLPEN` writer - DTSLPEN
pub type DTSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPTIM2LPEN
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM3LPEN
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPTIM4LPEN
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPTIM5LPEN
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SAI4LPEN
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SYSCFGLPEN
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - VREFLPEN
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - DTSLPEN
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_APB3LPENCLRR")
            .field("lptim2lpen", &self.lptim2lpen())
            .field("lptim3lpen", &self.lptim3lpen())
            .field("lptim4lpen", &self.lptim4lpen())
            .field("lptim5lpen", &self.lptim5lpen())
            .field("sai4lpen", &self.sai4lpen())
            .field("syscfglpen", &self.syscfglpen())
            .field("vreflpen", &self.vreflpen())
            .field("dtslpen", &self.dtslpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM2LPEN
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<MP_APB3LPENCLRRrs> {
        LPTIM2LPEN_W::new(self, 0)
    }
    ///Bit 1 - LPTIM3LPEN
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<MP_APB3LPENCLRRrs> {
        LPTIM3LPEN_W::new(self, 1)
    }
    ///Bit 2 - LPTIM4LPEN
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<MP_APB3LPENCLRRrs> {
        LPTIM4LPEN_W::new(self, 2)
    }
    ///Bit 3 - LPTIM5LPEN
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<MP_APB3LPENCLRRrs> {
        LPTIM5LPEN_W::new(self, 3)
    }
    ///Bit 8 - SAI4LPEN
    #[inline(always)]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W<MP_APB3LPENCLRRrs> {
        SAI4LPEN_W::new(self, 8)
    }
    ///Bit 11 - SYSCFGLPEN
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<MP_APB3LPENCLRRrs> {
        SYSCFGLPEN_W::new(self, 11)
    }
    ///Bit 13 - VREFLPEN
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<MP_APB3LPENCLRRrs> {
        VREFLPEN_W::new(self, 13)
    }
    ///Bit 16 - DTSLPEN
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<MP_APB3LPENCLRRrs> {
        DTSLPEN_W::new(self, 16)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb3lpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb3lpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_APB3LPENCLRR)*/
pub struct MP_APB3LPENCLRRrs;
impl crate::RegisterSpec for MP_APB3LPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_apb3lpenclrr::R`](R) reader structure
impl crate::Readable for MP_APB3LPENCLRRrs {}
///`write(|w| ..)` method takes [`mp_apb3lpenclrr::W`](W) writer structure
impl crate::Writable for MP_APB3LPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_APB3LPENCLRR to value 0x0003_290f
impl crate::Resettable for MP_APB3LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0003_290f;
}
