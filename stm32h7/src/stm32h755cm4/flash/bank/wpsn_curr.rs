///Register `WPSN_CURR` reader
pub type R = crate::R<WPSN_CURRrs>;
///Register `WPSN_CURR` writer
pub type W = crate::W<WPSN_CURRrs>;
///Field `WRPSn` reader - Bank 1 sector write protection option status byte
pub type WRPSN_R = crate::FieldReader;
///Field `WRPSn` writer - Bank 1 sector write protection option status byte
pub type WRPSN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Bank 1 sector write protection option status byte
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPSN_CURR")
            .field("wrpsn", &self.wrpsn())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Bank 1 sector write protection option status byte
    #[inline(always)]
    pub fn wrpsn(&mut self) -> WRPSN_W<'_, WPSN_CURRrs> {
        WRPSN_W::new(self, 0)
    }
}
/**FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_curr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_curr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WPSN_CURRrs;
impl crate::RegisterSpec for WPSN_CURRrs {
    type Ux = u32;
}
///`read()` method returns [`wpsn_curr::R`](R) reader structure
impl crate::Readable for WPSN_CURRrs {}
///`write(|w| ..)` method takes [`wpsn_curr::W`](W) writer structure
impl crate::Writable for WPSN_CURRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPSN_CURR to value 0
impl crate::Resettable for WPSN_CURRrs {}
