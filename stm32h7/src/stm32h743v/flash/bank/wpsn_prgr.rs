///Register `WPSN_PRGR` reader
pub type R = crate::R<WPSN_PRGRrs>;
///Register `WPSN_PRGR` writer
pub type W = crate::W<WPSN_PRGRrs>;
///Field `WRPSn` reader - Bank 1 sector write protection configuration byte
pub type WRPSN_R = crate::FieldReader;
///Field `WRPSn` writer - Bank 1 sector write protection configuration byte
pub type WRPSN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Bank 1 sector write protection configuration byte
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPSN_PRGR")
            .field("wrpsn", &self.wrpsn())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Bank 1 sector write protection configuration byte
    #[inline(always)]
    #[must_use]
    pub fn wrpsn(&mut self) -> WRPSN_W<WPSN_PRGRrs> {
        WRPSN_W::new(self, 0)
    }
}
/**FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_prgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WPSN_PRGRrs;
impl crate::RegisterSpec for WPSN_PRGRrs {
    type Ux = u32;
}
///`read()` method returns [`wpsn_prgr::R`](R) reader structure
impl crate::Readable for WPSN_PRGRrs {}
///`write(|w| ..)` method takes [`wpsn_prgr::W`](W) writer structure
impl crate::Writable for WPSN_PRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WPSN_PRGR to value 0
impl crate::Resettable for WPSN_PRGRrs {
    const RESET_VALUE: u32 = 0;
}
