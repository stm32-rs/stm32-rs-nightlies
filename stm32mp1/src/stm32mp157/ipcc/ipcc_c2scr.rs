///Register `IPCC_C2SCR` reader
pub type R = crate::R<IPCC_C2SCRrs>;
///Register `IPCC_C2SCR` writer
pub type W = crate::W<IPCC_C2SCRrs>;
///Field `CHxC` reader - CHxC
pub type CHX_C_R = crate::FieldReader;
///Field `CHxC` writer - CHxC
pub type CHX_C_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CHxS` reader - CHxS
pub type CHX_S_R = crate::FieldReader;
///Field `CHxS` writer - CHxS
pub type CHX_S_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - CHxC
    #[inline(always)]
    pub fn chx_c(&self) -> CHX_C_R {
        CHX_C_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:21 - CHxS
    #[inline(always)]
    pub fn chx_s(&self) -> CHX_S_R {
        CHX_S_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCC_C2SCR")
            .field("chx_c", &self.chx_c())
            .field("chx_s", &self.chx_s())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - CHxC
    #[inline(always)]
    #[must_use]
    pub fn chx_c(&mut self) -> CHX_C_W<IPCC_C2SCRrs> {
        CHX_C_W::new(self, 0)
    }
    ///Bits 16:21 - CHxS
    #[inline(always)]
    #[must_use]
    pub fn chx_s(&mut self) -> CHX_S_W<IPCC_C2SCRrs> {
        CHX_S_W::new(self, 16)
    }
}
/**Reading this register will always return 0x0000 0000.

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c2scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C2SCR)*/
pub struct IPCC_C2SCRrs;
impl crate::RegisterSpec for IPCC_C2SCRrs {
    type Ux = u32;
}
///`read()` method returns [`ipcc_c2scr::R`](R) reader structure
impl crate::Readable for IPCC_C2SCRrs {}
///`write(|w| ..)` method takes [`ipcc_c2scr::W`](W) writer structure
impl crate::Writable for IPCC_C2SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPCC_C2SCR to value 0
impl crate::Resettable for IPCC_C2SCRrs {
    const RESET_VALUE: u32 = 0;
}
