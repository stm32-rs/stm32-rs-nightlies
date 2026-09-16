///Register `PLL3CFGR1` reader
pub type R = crate::R<PLL3CFGR1rs>;
///Register `PLL3CFGR1` writer
pub type W = crate::W<PLL3CFGR1rs>;
///Field `DIVN` reader - DIVN
pub type DIVN_R = crate::FieldReader<u16>;
///Field `DIVN` writer - DIVN
pub type DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DIVM3` reader - DIVM3
pub type DIVM3_R = crate::FieldReader;
///Field `DIVM3` writer - DIVM3
pub type DIVM3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `IFRGE` reader - IFRGE
pub type IFRGE_R = crate::FieldReader;
///Field `IFRGE` writer - IFRGE
pub type IFRGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:21 - DIVM3
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:25 - IFRGE
    #[inline(always)]
    pub fn ifrge(&self) -> IFRGE_R {
        IFRGE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3CFGR1")
            .field("divn", &self.divn())
            .field("divm3", &self.divm3())
            .field("ifrge", &self.ifrge())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W<'_, PLL3CFGR1rs> {
        DIVN_W::new(self, 0)
    }
    ///Bits 16:21 - DIVM3
    #[inline(always)]
    pub fn divm3(&mut self) -> DIVM3_W<'_, PLL3CFGR1rs> {
        DIVM3_W::new(self, 16)
    }
    ///Bits 24:25 - IFRGE
    #[inline(always)]
    pub fn ifrge(&mut self) -> IFRGE_W<'_, PLL3CFGR1rs> {
        IFRGE_W::new(self, 24)
    }
}
/**This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL3CFGR1)*/
pub struct PLL3CFGR1rs;
impl crate::RegisterSpec for PLL3CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll3cfgr1::R`](R) reader structure
impl crate::Readable for PLL3CFGR1rs {}
///`write(|w| ..)` method takes [`pll3cfgr1::W`](W) writer structure
impl crate::Writable for PLL3CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3CFGR1 to value 0x0001_0031
impl crate::Resettable for PLL3CFGR1rs {
    const RESET_VALUE: u32 = 0x0001_0031;
}
