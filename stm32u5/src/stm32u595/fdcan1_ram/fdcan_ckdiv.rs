///Register `FDCAN_CKDIV` reader
pub type R = crate::R<FDCAN_CKDIVrs>;
///Register `FDCAN_CKDIV` writer
pub type W = crate::W<FDCAN_CKDIVrs>;
///Field `PDIV` reader - PDIV
pub type PDIV_R = crate::FieldReader;
///Field `PDIV` writer - PDIV
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - PDIV
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CKDIV")
            .field("pdiv", &self.pdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - PDIV
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<FDCAN_CKDIVrs> {
        PDIV_W::new(self, 0)
    }
}
/**FDCAN CFG clock divider register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ckdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ckdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FDCAN1_RAM:FDCAN_CKDIV)*/
pub struct FDCAN_CKDIVrs;
impl crate::RegisterSpec for FDCAN_CKDIVrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ckdiv::R`](R) reader structure
impl crate::Readable for FDCAN_CKDIVrs {}
///`write(|w| ..)` method takes [`fdcan_ckdiv::W`](W) writer structure
impl crate::Writable for FDCAN_CKDIVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_CKDIV to value 0
impl crate::Resettable for FDCAN_CKDIVrs {
    const RESET_VALUE: u32 = 0;
}
