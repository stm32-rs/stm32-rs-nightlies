///Register `PLL3DIVR` reader
pub type R = crate::R<PLL3DIVRrs>;
///Register `PLL3DIVR` writer
pub type W = crate::W<PLL3DIVRrs>;
///Field `DIVN3` reader - Multiplication factor for PLL1 VCO
pub type DIVN3_R = crate::FieldReader<u16>;
///Field `DIVN3` writer - Multiplication factor for PLL1 VCO
pub type DIVN3_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DIVP3` reader - PLL DIVP division factor
pub type DIVP3_R = crate::FieldReader;
///Field `DIVP3` writer - PLL DIVP division factor
pub type DIVP3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVQ3` reader - PLL DIVQ division factor
pub type DIVQ3_R = crate::FieldReader;
///Field `DIVQ3` writer - PLL DIVQ division factor
pub type DIVQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
///Field `DIVR3` reader - PLL DIVR division factor
pub type DIVR3_R = crate::FieldReader;
///Field `DIVR3` writer - PLL DIVR division factor
pub type DIVR3_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn divn3(&self) -> DIVN3_R {
        DIVN3_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL DIVP division factor
    #[inline(always)]
    pub fn divp3(&self) -> DIVP3_R {
        DIVP3_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL DIVQ division factor
    #[inline(always)]
    pub fn divq3(&self) -> DIVQ3_R {
        DIVQ3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL DIVR division factor
    #[inline(always)]
    pub fn divr3(&self) -> DIVR3_R {
        DIVR3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3DIVR")
            .field("divn3", &self.divn3())
            .field("divp3", &self.divp3())
            .field("divq3", &self.divq3())
            .field("divr3", &self.divr3())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn divn3(&mut self) -> DIVN3_W<'_, PLL3DIVRrs> {
        DIVN3_W::new(self, 0)
    }
    ///Bits 9:15 - PLL DIVP division factor
    #[inline(always)]
    pub fn divp3(&mut self) -> DIVP3_W<'_, PLL3DIVRrs> {
        DIVP3_W::new(self, 9)
    }
    ///Bits 16:22 - PLL DIVQ division factor
    #[inline(always)]
    pub fn divq3(&mut self) -> DIVQ3_W<'_, PLL3DIVRrs> {
        DIVQ3_W::new(self, 16)
    }
    ///Bits 24:30 - PLL DIVR division factor
    #[inline(always)]
    pub fn divr3(&mut self) -> DIVR3_W<'_, PLL3DIVRrs> {
        DIVR3_W::new(self, 24)
    }
}
/**RCC PLL3 Dividers Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pll3divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#RCC:PLL3DIVR)*/
pub struct PLL3DIVRrs;
impl crate::RegisterSpec for PLL3DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`pll3divr::R`](R) reader structure
impl crate::Readable for PLL3DIVRrs {}
///`write(|w| ..)` method takes [`pll3divr::W`](W) writer structure
impl crate::Writable for PLL3DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3DIVR to value 0x0101_0280
impl crate::Resettable for PLL3DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
