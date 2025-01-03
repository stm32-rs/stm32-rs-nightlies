///Register `PWR_PDCRF` reader
pub type R = crate::R<PWR_PDCRFrs>;
///Register `PWR_PDCRF` writer
pub type W = crate::W<PWR_PDCRFrs>;
/**Field `PD0` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD0_R = crate::BitReader;
/**Field `PD0` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD1` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD1_R = crate::BitReader;
/**Field `PD1` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD2` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD2_R = crate::BitReader;
/**Field `PD2` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD3` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD3_R = crate::BitReader;
/**Field `PD3` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
when APC bit is set in PWR_CR3 register.*/
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 0 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    /**Bit 1 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    /**Bit 2 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    /**Bit 3 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_PDCRF")
            .field("pd0", &self.pd0())
            .field("pd1", &self.pd1())
            .field("pd2", &self.pd2())
            .field("pd3", &self.pd3())
            .finish()
    }
}
impl W {
    /**Bit 0 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<PWR_PDCRFrs> {
        PD0_W::new(self, 0)
    }
    /**Bit 1 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<PWR_PDCRFrs> {
        PD1_W::new(self, 1)
    }
    /**Bit 2 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<PWR_PDCRFrs> {
        PD2_W::new(self, 2)
    }
    /**Bit 3 - Port F pull-down bit y When set, this bit activates the pull-down on PH\[y\]
    when APC bit is set in PWR_CR3 register.*/
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<PWR_PDCRFrs> {
        PD3_W::new(self, 3)
    }
}
/**Power Port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#PWR:PWR_PDCRF)*/
pub struct PWR_PDCRFrs;
impl crate::RegisterSpec for PWR_PDCRFrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_pdcrf::R`](R) reader structure
impl crate::Readable for PWR_PDCRFrs {}
///`write(|w| ..)` method takes [`pwr_pdcrf::W`](W) writer structure
impl crate::Writable for PWR_PDCRFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_PDCRF to value 0
impl crate::Resettable for PWR_PDCRFrs {
    const RESET_VALUE: u32 = 0;
}
