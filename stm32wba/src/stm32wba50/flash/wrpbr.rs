///Register `WRPBR` reader
pub type R = crate::R<WRPBRrs>;
///Register `WRPBR` writer
pub type W = crate::W<WRPBRrs>;
///Field `WRPB_PSTRT` reader - WRP area B start page This field contains the first page of the WRP area B. Note that bit 6 is reserved on STM32WBAxEx devices.
pub type WRPB_PSTRT_R = crate::FieldReader;
///Field `WRPB_PSTRT` writer - WRP area B start page This field contains the first page of the WRP area B. Note that bit 6 is reserved on STM32WBAxEx devices.
pub type WRPB_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRPB_PEND` reader - WRP area B end page This field contains the last page of the WRP area B. Note that bit 22 is reserved on STM32WBAxEx devices.
pub type WRPB_PEND_R = crate::FieldReader;
///Field `WRPB_PEND` writer - WRP area B end page This field contains the last page of the WRP area B. Note that bit 22 is reserved on STM32WBAxEx devices.
pub type WRPB_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `UNLOCK` reader - WPR area B unlock
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - WPR area B unlock
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - WRP area B start page This field contains the first page of the WRP area B. Note that bit 6 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpb_pstrt(&self) -> WRPB_PSTRT_R {
        WRPB_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP area B end page This field contains the last page of the WRP area B. Note that bit 22 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpb_pend(&self) -> WRPB_PEND_R {
        WRPB_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - WPR area B unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPBR")
            .field("wrpb_pstrt", &self.wrpb_pstrt())
            .field("wrpb_pend", &self.wrpb_pend())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - WRP area B start page This field contains the first page of the WRP area B. Note that bit 6 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpb_pstrt(&mut self) -> WRPB_PSTRT_W<'_, WRPBRrs> {
        WRPB_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - WRP area B end page This field contains the last page of the WRP area B. Note that bit 22 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpb_pend(&mut self) -> WRPB_PEND_W<'_, WRPBRrs> {
        WRPB_PEND_W::new(self, 16)
    }
    ///Bit 31 - WPR area B unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<'_, WRPBRrs> {
        UNLOCK_W::new(self, 31)
    }
}
/**FLASH WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:WRPBR)*/
pub struct WRPBRrs;
impl crate::RegisterSpec for WRPBRrs {
    type Ux = u32;
}
///`read()` method returns [`wrpbr::R`](R) reader structure
impl crate::Readable for WRPBRrs {}
///`write(|w| ..)` method takes [`wrpbr::W`](W) writer structure
impl crate::Writable for WRPBRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPBR to value 0x0f00_ff00
impl crate::Resettable for WRPBRrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
