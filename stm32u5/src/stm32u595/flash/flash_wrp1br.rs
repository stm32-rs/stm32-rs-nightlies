///Register `FLASH_WRP1BR` reader
pub type R = crate::R<FLASH_WRP1BRrs>;
///Register `FLASH_WRP1BR` writer
pub type W = crate::W<FLASH_WRP1BRrs>;
///Field `WRP1B_PSTRT` reader - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
pub type WRP1B_PSTRT_R = crate::FieldReader;
///Field `WRP1B_PSTRT` writer - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
pub type WRP1B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WRP1B_PEND` reader - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
pub type WRP1B_PEND_R = crate::FieldReader;
///Field `WRP1B_PEND` writer - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
pub type WRP1B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UNLOCK` reader - Bank 1 WPR second area B unlock
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - Bank 1 WPR second area B unlock
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
    #[inline(always)]
    pub fn wrp1b_pstrt(&self) -> WRP1B_PSTRT_R {
        WRP1B_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
    #[inline(always)]
    pub fn wrp1b_pend(&self) -> WRP1B_PEND_R {
        WRP1B_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - Bank 1 WPR second area B unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_WRP1BR")
            .field("wrp1b_pstrt", &self.wrp1b_pstrt())
            .field("wrp1b_pend", &self.wrp1b_pend())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_pstrt(&mut self) -> WRP1B_PSTRT_W<FLASH_WRP1BRrs> {
        WRP1B_PSTRT_W::new(self, 0)
    }
    ///Bits 16:23 - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_pend(&mut self) -> WRP1B_PEND_W<FLASH_WRP1BRrs> {
        WRP1B_PEND_W::new(self, 16)
    }
    ///Bit 31 - Bank 1 WPR second area B unlock
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<FLASH_WRP1BRrs> {
        UNLOCK_W::new(self, 31)
    }
}
/**FLASH WRP1 area B address register

You can [`read`](crate::Reg::read) this register and get [`flash_wrp1br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_wrp1br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FLASH:FLASH_WRP1BR)*/
pub struct FLASH_WRP1BRrs;
impl crate::RegisterSpec for FLASH_WRP1BRrs {
    type Ux = u32;
}
///`read()` method returns [`flash_wrp1br::R`](R) reader structure
impl crate::Readable for FLASH_WRP1BRrs {}
///`write(|w| ..)` method takes [`flash_wrp1br::W`](W) writer structure
impl crate::Writable for FLASH_WRP1BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_WRP1BR to value 0x0f00_ff00
impl crate::Resettable for FLASH_WRP1BRrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
