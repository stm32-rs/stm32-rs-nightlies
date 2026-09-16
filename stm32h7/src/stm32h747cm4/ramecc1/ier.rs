///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `GIE` reader - Global interrupt enable
pub type GIE_R = crate::BitReader;
///Field `GIE` writer - Global interrupt enable
pub type GIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GECCSEIE` reader - Global ECC single error interrupt enable
pub type GECCSEIE_R = crate::BitReader;
///Field `GECCSEIE` writer - Global ECC single error interrupt enable
pub type GECCSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GECCDEIE` reader - Global ECC double error interrupt enable
pub type GECCDEIE_R = crate::BitReader;
///Field `GECCDEIE` writer - Global ECC double error interrupt enable
pub type GECCDEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GECCDEBWIE` reader - Global ECC double error on byte write (BW) interrupt enable
pub type GECCDEBWIE_R = crate::BitReader;
///Field `GECCDEBWIE` writer - Global ECC double error on byte write (BW) interrupt enable
pub type GECCDEBWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Global interrupt enable
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Global ECC single error interrupt enable
    #[inline(always)]
    pub fn geccseie(&self) -> GECCSEIE_R {
        GECCSEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Global ECC double error interrupt enable
    #[inline(always)]
    pub fn geccdeie(&self) -> GECCDEIE_R {
        GECCDEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Global ECC double error on byte write (BW) interrupt enable
    #[inline(always)]
    pub fn geccdebwie(&self) -> GECCDEBWIE_R {
        GECCDEBWIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("gie", &self.gie())
            .field("geccseie", &self.geccseie())
            .field("geccdeie", &self.geccdeie())
            .field("geccdebwie", &self.geccdebwie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Global interrupt enable
    #[inline(always)]
    pub fn gie(&mut self) -> GIE_W<'_, IERrs> {
        GIE_W::new(self, 0)
    }
    ///Bit 1 - Global ECC single error interrupt enable
    #[inline(always)]
    pub fn geccseie(&mut self) -> GECCSEIE_W<'_, IERrs> {
        GECCSEIE_W::new(self, 1)
    }
    ///Bit 2 - Global ECC double error interrupt enable
    #[inline(always)]
    pub fn geccdeie(&mut self) -> GECCDEIE_W<'_, IERrs> {
        GECCDEIE_W::new(self, 2)
    }
    ///Bit 3 - Global ECC double error on byte write (BW) interrupt enable
    #[inline(always)]
    pub fn geccdebwie(&mut self) -> GECCDEBWIE_W<'_, IERrs> {
        GECCDEBWIE_W::new(self, 3)
    }
}
/**RAMECC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#RAMECC1:IER)*/
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
