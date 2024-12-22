///Register `PDCRD` reader
pub type R = crate::R<PDCRDrs>;
///Register `PDCRD` writer
pub type W = crate::W<PDCRDrs>;
/**Field `PD0` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD0_R = crate::BitReader;
/**Field `PD0` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD1` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD1_R = crate::BitReader;
/**Field `PD1` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD2` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD2_R = crate::BitReader;
/**Field `PD2` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD3` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD3_R = crate::BitReader;
/**Field `PD3` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
I/O.*/
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 0 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    /**Bit 1 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    /**Bit 2 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    /**Bit 3 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRD")
            .field("pd0", &self.pd0())
            .field("pd1", &self.pd1())
            .field("pd2", &self.pd2())
            .field("pd3", &self.pd3())
            .finish()
    }
}
impl W {
    /**Bit 0 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<PDCRDrs> {
        PD0_W::new(self, 0)
    }
    /**Bit 1 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<PDCRDrs> {
        PD1_W::new(self, 1)
    }
    /**Bit 2 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<PDCRDrs> {
        PD2_W::new(self, 2)
    }
    /**Bit 3 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\[i\]
    I/O.*/
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<PDCRDrs> {
        PD3_W::new(self, 3)
    }
}
/**PWR Port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PDCRD)*/
pub struct PDCRDrs;
impl crate::RegisterSpec for PDCRDrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrd::R`](R) reader structure
impl crate::Readable for PDCRDrs {}
///`write(|w| ..)` method takes [`pdcrd::W`](W) writer structure
impl crate::Writable for PDCRDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PDCRD to value 0
impl crate::Resettable for PDCRDrs {
    const RESET_VALUE: u32 = 0;
}
