///Register `EECR2` reader
pub type R = crate::R<EECR2rs>;
///Register `EECR2` writer
pub type W = crate::W<EECR2rs>;
///Field `EE6SRC` reader - EE6SRC
pub type EE6SRC_R = crate::FieldReader;
///Field `EE6SRC` writer - EE6SRC
pub type EE6SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE6POL` reader - EE6POL
pub type EE6POL_R = crate::BitReader;
///Field `EE6POL` writer - EE6POL
pub type EE6POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE6SNS` reader - EE6SNS
pub type EE6SNS_R = crate::FieldReader;
///Field `EE6SNS` writer - EE6SNS
pub type EE6SNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE7SRC` reader - EE7SRC
pub type EE7SRC_R = crate::FieldReader;
///Field `EE7SRC` writer - EE7SRC
pub type EE7SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE7POL` reader - EE7POL
pub type EE7POL_R = crate::BitReader;
///Field `EE7POL` writer - EE7POL
pub type EE7POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE7SNS` reader - EE7SNS
pub type EE7SNS_R = crate::FieldReader;
///Field `EE7SNS` writer - EE7SNS
pub type EE7SNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE8SRC` reader - EE8SRC
pub type EE8SRC_R = crate::FieldReader;
///Field `EE8SRC` writer - EE8SRC
pub type EE8SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE8POL` reader - EE8POL
pub type EE8POL_R = crate::BitReader;
///Field `EE8POL` writer - EE8POL
pub type EE8POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE8SNS` reader - EE8SNS
pub type EE8SNS_R = crate::FieldReader;
///Field `EE8SNS` writer - EE8SNS
pub type EE8SNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE9SRC` reader - EE9SRC
pub type EE9SRC_R = crate::FieldReader;
///Field `EE9SRC` writer - EE9SRC
pub type EE9SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE9POL` reader - EE9POL
pub type EE9POL_R = crate::BitReader;
///Field `EE9POL` writer - EE9POL
pub type EE9POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE9SNS` reader - EE9SNS
pub type EE9SNS_R = crate::FieldReader;
///Field `EE9SNS` writer - EE9SNS
pub type EE9SNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE10SRC` reader - EE10SRC
pub type EE10SRC_R = crate::FieldReader;
///Field `EE10SRC` writer - EE10SRC
pub type EE10SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EE10POL` reader - EE10POL
pub type EE10POL_R = crate::BitReader;
///Field `EE10POL` writer - EE10POL
pub type EE10POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE10SNS` reader - EE10SNS
pub type EE10SNS_R = crate::FieldReader;
///Field `EE10SNS` writer - EE10SNS
pub type EE10SNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - EE6SRC
    #[inline(always)]
    pub fn ee6src(&self) -> EE6SRC_R {
        EE6SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - EE6POL
    #[inline(always)]
    pub fn ee6pol(&self) -> EE6POL_R {
        EE6POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - EE6SNS
    #[inline(always)]
    pub fn ee6sns(&self) -> EE6SNS_R {
        EE6SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - EE7SRC
    #[inline(always)]
    pub fn ee7src(&self) -> EE7SRC_R {
        EE7SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - EE7POL
    #[inline(always)]
    pub fn ee7pol(&self) -> EE7POL_R {
        EE7POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - EE7SNS
    #[inline(always)]
    pub fn ee7sns(&self) -> EE7SNS_R {
        EE7SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 12:13 - EE8SRC
    #[inline(always)]
    pub fn ee8src(&self) -> EE8SRC_R {
        EE8SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - EE8POL
    #[inline(always)]
    pub fn ee8pol(&self) -> EE8POL_R {
        EE8POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:16 - EE8SNS
    #[inline(always)]
    pub fn ee8sns(&self) -> EE8SNS_R {
        EE8SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 18:19 - EE9SRC
    #[inline(always)]
    pub fn ee9src(&self) -> EE9SRC_R {
        EE9SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - EE9POL
    #[inline(always)]
    pub fn ee9pol(&self) -> EE9POL_R {
        EE9POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - EE9SNS
    #[inline(always)]
    pub fn ee9sns(&self) -> EE9SNS_R {
        EE9SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 24:25 - EE10SRC
    #[inline(always)]
    pub fn ee10src(&self) -> EE10SRC_R {
        EE10SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - EE10POL
    #[inline(always)]
    pub fn ee10pol(&self) -> EE10POL_R {
        EE10POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - EE10SNS
    #[inline(always)]
    pub fn ee10sns(&self) -> EE10SNS_R {
        EE10SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR2")
            .field("ee6src", &self.ee6src())
            .field("ee6pol", &self.ee6pol())
            .field("ee6sns", &self.ee6sns())
            .field("ee7src", &self.ee7src())
            .field("ee7pol", &self.ee7pol())
            .field("ee7sns", &self.ee7sns())
            .field("ee8src", &self.ee8src())
            .field("ee8pol", &self.ee8pol())
            .field("ee8sns", &self.ee8sns())
            .field("ee9src", &self.ee9src())
            .field("ee9pol", &self.ee9pol())
            .field("ee9sns", &self.ee9sns())
            .field("ee10src", &self.ee10src())
            .field("ee10pol", &self.ee10pol())
            .field("ee10sns", &self.ee10sns())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - EE6SRC
    #[inline(always)]
    #[must_use]
    pub fn ee6src(&mut self) -> EE6SRC_W<EECR2rs> {
        EE6SRC_W::new(self, 0)
    }
    ///Bit 2 - EE6POL
    #[inline(always)]
    #[must_use]
    pub fn ee6pol(&mut self) -> EE6POL_W<EECR2rs> {
        EE6POL_W::new(self, 2)
    }
    ///Bits 3:4 - EE6SNS
    #[inline(always)]
    #[must_use]
    pub fn ee6sns(&mut self) -> EE6SNS_W<EECR2rs> {
        EE6SNS_W::new(self, 3)
    }
    ///Bits 6:7 - EE7SRC
    #[inline(always)]
    #[must_use]
    pub fn ee7src(&mut self) -> EE7SRC_W<EECR2rs> {
        EE7SRC_W::new(self, 6)
    }
    ///Bit 8 - EE7POL
    #[inline(always)]
    #[must_use]
    pub fn ee7pol(&mut self) -> EE7POL_W<EECR2rs> {
        EE7POL_W::new(self, 8)
    }
    ///Bits 9:10 - EE7SNS
    #[inline(always)]
    #[must_use]
    pub fn ee7sns(&mut self) -> EE7SNS_W<EECR2rs> {
        EE7SNS_W::new(self, 9)
    }
    ///Bits 12:13 - EE8SRC
    #[inline(always)]
    #[must_use]
    pub fn ee8src(&mut self) -> EE8SRC_W<EECR2rs> {
        EE8SRC_W::new(self, 12)
    }
    ///Bit 14 - EE8POL
    #[inline(always)]
    #[must_use]
    pub fn ee8pol(&mut self) -> EE8POL_W<EECR2rs> {
        EE8POL_W::new(self, 14)
    }
    ///Bits 15:16 - EE8SNS
    #[inline(always)]
    #[must_use]
    pub fn ee8sns(&mut self) -> EE8SNS_W<EECR2rs> {
        EE8SNS_W::new(self, 15)
    }
    ///Bits 18:19 - EE9SRC
    #[inline(always)]
    #[must_use]
    pub fn ee9src(&mut self) -> EE9SRC_W<EECR2rs> {
        EE9SRC_W::new(self, 18)
    }
    ///Bit 20 - EE9POL
    #[inline(always)]
    #[must_use]
    pub fn ee9pol(&mut self) -> EE9POL_W<EECR2rs> {
        EE9POL_W::new(self, 20)
    }
    ///Bits 21:22 - EE9SNS
    #[inline(always)]
    #[must_use]
    pub fn ee9sns(&mut self) -> EE9SNS_W<EECR2rs> {
        EE9SNS_W::new(self, 21)
    }
    ///Bits 24:25 - EE10SRC
    #[inline(always)]
    #[must_use]
    pub fn ee10src(&mut self) -> EE10SRC_W<EECR2rs> {
        EE10SRC_W::new(self, 24)
    }
    ///Bit 26 - EE10POL
    #[inline(always)]
    #[must_use]
    pub fn ee10pol(&mut self) -> EE10POL_W<EECR2rs> {
        EE10POL_W::new(self, 26)
    }
    ///Bits 27:28 - EE10SNS
    #[inline(always)]
    #[must_use]
    pub fn ee10sns(&mut self) -> EE10SNS_W<EECR2rs> {
        EE10SNS_W::new(self, 27)
    }
}
/**Timer External Event Control Register 2

You can [`read`](crate::Reg::read) this register and get [`eecr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_Common:EECR2)*/
pub struct EECR2rs;
impl crate::RegisterSpec for EECR2rs {
    type Ux = u32;
}
///`read()` method returns [`eecr2::R`](R) reader structure
impl crate::Readable for EECR2rs {}
///`write(|w| ..)` method takes [`eecr2::W`](W) writer structure
impl crate::Writable for EECR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EECR2 to value 0
impl crate::Resettable for EECR2rs {
    const RESET_VALUE: u32 = 0;
}
