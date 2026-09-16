///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
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
        f.debug_struct("IER")
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
    pub fn cmpmie(&mut self) -> CMPMIE_W<'_, IERrs> {
        CMPMIE_W::new(self, 0)
    }
    ///Bit 1 - ARRMIE
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<'_, IERrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - EXTTRIGIE
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<'_, IERrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 3 - CMPOKIE
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<'_, IERrs> {
        CMPOKIE_W::new(self, 3)
    }
    ///Bit 4 - ARROKIE
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<'_, IERrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - UPIE
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<'_, IERrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - DOWNIE
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<'_, IERrs> {
        DOWNIE_W::new(self, 6)
    }
}
/**LPTIM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
