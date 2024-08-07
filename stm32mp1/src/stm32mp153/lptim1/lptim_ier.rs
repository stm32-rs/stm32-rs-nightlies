///Register `LPTIM_IER` reader
pub type R = crate::R<LPTIM_IERrs>;
///Register `LPTIM_IER` writer
pub type W = crate::W<LPTIM_IERrs>;
///Field `CMPMIE` reader - CMPMIE
pub type CMPMIE_R = crate::BitReader;
///Field `CMPMIE` writer - CMPMIE
pub type CMPMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMIE` reader - ARRMIE
pub type ARRMIE_R = crate::BitReader;
///Field `ARRMIE` writer - ARRMIE
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGIE` reader - EXTTRIGIE
pub type EXTTRIGIE_R = crate::BitReader;
///Field `EXTTRIGIE` writer - EXTTRIGIE
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPOKIE` reader - CMPOKIE
pub type CMPOKIE_R = crate::BitReader;
///Field `CMPOKIE` writer - CMPOKIE
pub type CMPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKIE` reader - ARROKIE
pub type ARROKIE_R = crate::BitReader;
///Field `ARROKIE` writer - ARROKIE
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPIE` reader - UPIE
pub type UPIE_R = crate::BitReader;
///Field `UPIE` writer - UPIE
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNIE` reader - DOWNIE
pub type DOWNIE_R = crate::BitReader;
///Field `DOWNIE` writer - DOWNIE
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CMPMIE
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ARRMIE
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EXTTRIGIE
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CMPOKIE
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ARROKIE
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UPIE
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DOWNIE
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_IER")
            .field("cmpmie", &self.cmpmie())
            .field("arrmie", &self.arrmie())
            .field("exttrigie", &self.exttrigie())
            .field("cmpokie", &self.cmpokie())
            .field("arrokie", &self.arrokie())
            .field("upie", &self.upie())
            .field("downie", &self.downie())
            .finish()
    }
}
impl W {
    ///Bit 0 - CMPMIE
    #[inline(always)]
    #[must_use]
    pub fn cmpmie(&mut self) -> CMPMIE_W<LPTIM_IERrs> {
        CMPMIE_W::new(self, 0)
    }
    ///Bit 1 - ARRMIE
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<LPTIM_IERrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - EXTTRIGIE
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<LPTIM_IERrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 3 - CMPOKIE
    #[inline(always)]
    #[must_use]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<LPTIM_IERrs> {
        CMPOKIE_W::new(self, 3)
    }
    ///Bit 4 - ARROKIE
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<LPTIM_IERrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - UPIE
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<LPTIM_IERrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - DOWNIE
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<LPTIM_IERrs> {
        DOWNIE_W::new(self, 6)
    }
}
/**LPTIM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`lptim_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LPTIM1:LPTIM_IER)*/
pub struct LPTIM_IERrs;
impl crate::RegisterSpec for LPTIM_IERrs {
    type Ux = u32;
}
///`read()` method returns [`lptim_ier::R`](R) reader structure
impl crate::Readable for LPTIM_IERrs {}
///`write(|w| ..)` method takes [`lptim_ier::W`](W) writer structure
impl crate::Writable for LPTIM_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM_IER to value 0
impl crate::Resettable for LPTIM_IERrs {
    const RESET_VALUE: u32 = 0;
}
