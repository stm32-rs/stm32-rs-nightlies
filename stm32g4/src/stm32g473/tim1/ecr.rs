///Register `ECR` reader
pub type R = crate::R<ECRrs>;
///Register `ECR` writer
pub type W = crate::W<ECRrs>;
///Field `IE` reader - Index Enable
pub type IE_R = crate::BitReader;
///Field `IE` writer - Index Enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDIR` reader - Index Direction
pub type IDIR_R = crate::FieldReader;
///Field `IDIR` writer - Index Direction
pub type IDIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IBLK` reader - Index Blanking
pub type IBLK_R = crate::FieldReader;
///Field `IBLK` writer - Index Blanking
pub type IBLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FIDX` reader - First Index
pub type FIDX_R = crate::BitReader;
///Field `FIDX` writer - First Index
pub type FIDX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPOS` reader - Index Positioning
pub type IPOS_R = crate::FieldReader;
///Field `IPOS` writer - Index Positioning
pub type IPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PW` reader - Pulse width
pub type PW_R = crate::FieldReader;
///Field `PW` writer - Pulse width
pub type PW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PWPRSC` reader - Pulse Width prescaler
pub type PWPRSC_R = crate::FieldReader;
///Field `PWPRSC` writer - Pulse Width prescaler
pub type PWPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Index Enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Index Direction
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Index Blanking
    #[inline(always)]
    pub fn iblk(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - First Index
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Index Positioning
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 16:23 - Pulse width
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - Pulse Width prescaler
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR")
            .field("ie", &self.ie())
            .field("idir", &self.idir())
            .field("iblk", &self.iblk())
            .field("fidx", &self.fidx())
            .field("ipos", &self.ipos())
            .field("pw", &self.pw())
            .field("pwprsc", &self.pwprsc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Index Enable
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<ECRrs> {
        IE_W::new(self, 0)
    }
    ///Bits 1:2 - Index Direction
    #[inline(always)]
    #[must_use]
    pub fn idir(&mut self) -> IDIR_W<ECRrs> {
        IDIR_W::new(self, 1)
    }
    ///Bits 3:4 - Index Blanking
    #[inline(always)]
    #[must_use]
    pub fn iblk(&mut self) -> IBLK_W<ECRrs> {
        IBLK_W::new(self, 3)
    }
    ///Bit 5 - First Index
    #[inline(always)]
    #[must_use]
    pub fn fidx(&mut self) -> FIDX_W<ECRrs> {
        FIDX_W::new(self, 5)
    }
    ///Bits 6:7 - Index Positioning
    #[inline(always)]
    #[must_use]
    pub fn ipos(&mut self) -> IPOS_W<ECRrs> {
        IPOS_W::new(self, 6)
    }
    ///Bits 16:23 - Pulse width
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<ECRrs> {
        PW_W::new(self, 16)
    }
    ///Bits 24:26 - Pulse Width prescaler
    #[inline(always)]
    #[must_use]
    pub fn pwprsc(&mut self) -> PWPRSC_W<ECRrs> {
        PWPRSC_W::new(self, 24)
    }
}
/**DMA control register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473xx.html#TIM1:ECR)*/
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
///`read()` method returns [`ecr::R`](R) reader structure
impl crate::Readable for ECRrs {}
///`write(|w| ..)` method takes [`ecr::W`](W) writer structure
impl crate::Writable for ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECRrs {
    const RESET_VALUE: u32 = 0;
}
