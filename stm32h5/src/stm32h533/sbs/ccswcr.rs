///Register `CCSWCR` reader
pub type R = crate::R<CCSWCRrs>;
///Register `CCSWCR` writer
pub type W = crate::W<CCSWCRrs>;
///Field `SW_ANSRC1` reader - NMOS compensation code for VDD power rails
pub type SW_ANSRC1_R = crate::FieldReader;
///Field `SW_ANSRC1` writer - NMOS compensation code for VDD power rails
pub type SW_ANSRC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SW_APSRC1` reader - PMOS compensation code for the VDD power rails
pub type SW_APSRC1_R = crate::FieldReader;
///Field `SW_APSRC1` writer - PMOS compensation code for the VDD power rails
pub type SW_APSRC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SW_ANSRC2` reader - NMOS compensation code for VDDIO power rails
pub type SW_ANSRC2_R = crate::FieldReader;
///Field `SW_ANSRC2` writer - NMOS compensation code for VDDIO power rails
pub type SW_ANSRC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SW_APSRC2` reader - PMOS compensation code for the Vless thansub>DDIOless than/sub> power rails
pub type SW_APSRC2_R = crate::FieldReader;
///Field `SW_APSRC2` writer - PMOS compensation code for the Vless thansub>DDIOless than/sub> power rails
pub type SW_APSRC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - NMOS compensation code for VDD power rails
    #[inline(always)]
    pub fn sw_ansrc1(&self) -> SW_ANSRC1_R {
        SW_ANSRC1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PMOS compensation code for the VDD power rails
    #[inline(always)]
    pub fn sw_apsrc1(&self) -> SW_APSRC1_R {
        SW_APSRC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - NMOS compensation code for VDDIO power rails
    #[inline(always)]
    pub fn sw_ansrc2(&self) -> SW_ANSRC2_R {
        SW_ANSRC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PMOS compensation code for the Vless thansub>DDIOless than/sub> power rails
    #[inline(always)]
    pub fn sw_apsrc2(&self) -> SW_APSRC2_R {
        SW_APSRC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCSWCR")
            .field("sw_ansrc1", &self.sw_ansrc1())
            .field("sw_apsrc1", &self.sw_apsrc1())
            .field("sw_ansrc2", &self.sw_ansrc2())
            .field("sw_apsrc2", &self.sw_apsrc2())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - NMOS compensation code for VDD power rails
    #[inline(always)]
    pub fn sw_ansrc1(&mut self) -> SW_ANSRC1_W<'_, CCSWCRrs> {
        SW_ANSRC1_W::new(self, 0)
    }
    ///Bits 4:7 - PMOS compensation code for the VDD power rails
    #[inline(always)]
    pub fn sw_apsrc1(&mut self) -> SW_APSRC1_W<'_, CCSWCRrs> {
        SW_APSRC1_W::new(self, 4)
    }
    ///Bits 8:11 - NMOS compensation code for VDDIO power rails
    #[inline(always)]
    pub fn sw_ansrc2(&mut self) -> SW_ANSRC2_W<'_, CCSWCRrs> {
        SW_ANSRC2_W::new(self, 8)
    }
    ///Bits 12:15 - PMOS compensation code for the Vless thansub>DDIOless than/sub> power rails
    #[inline(always)]
    pub fn sw_apsrc2(&mut self) -> SW_APSRC2_W<'_, CCSWCRrs> {
        SW_APSRC2_W::new(self, 12)
    }
}
/**SBS compensation cell for I/Os software code register

You can [`read`](crate::Reg::read) this register and get [`ccswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#SBS:CCSWCR)*/
pub struct CCSWCRrs;
impl crate::RegisterSpec for CCSWCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccswcr::R`](R) reader structure
impl crate::Readable for CCSWCRrs {}
///`write(|w| ..)` method takes [`ccswcr::W`](W) writer structure
impl crate::Writable for CCSWCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCSWCR to value 0x7878
impl crate::Resettable for CCSWCRrs {
    const RESET_VALUE: u32 = 0x7878;
}
