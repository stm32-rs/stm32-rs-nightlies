///Register `CDCFGR2` reader
pub type R = crate::R<CDCFGR2rs>;
///Register `CDCFGR2` writer
pub type W = crate::W<CDCFGR2rs>;
/**CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDPPRE1 {
    ///4: rcc_hclk divided by 2
    Div2 = 4,
    ///5: rcc_hclk divided by 4
    Div4 = 5,
    ///6: rcc_hclk divided by 8
    Div8 = 6,
    ///7: rcc_hclk divided by 16
    Div16 = 7,
    ///0: rcc_hclk not divided
    Div1 = 0,
}
impl From<CDPPRE1> for u8 {
    #[inline(always)]
    fn from(variant: CDPPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDPPRE1 {
    type Ux = u8;
}
impl crate::IsEnum for CDPPRE1 {}
///Field `CDPPRE1` reader - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)
pub type CDPPRE1_R = crate::FieldReader<CDPPRE1>;
impl CDPPRE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDPPRE1 {
        match self.bits {
            4 => CDPPRE1::Div2,
            5 => CDPPRE1::Div4,
            6 => CDPPRE1::Div8,
            7 => CDPPRE1::Div16,
            _ => CDPPRE1::Div1,
        }
    }
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CDPPRE1::Div2
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CDPPRE1::Div4
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CDPPRE1::Div8
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CDPPRE1::Div16
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), CDPPRE1::Div1)
    }
}
///Field `CDPPRE1` writer - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)
pub type CDPPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CDPPRE1, crate::Safe>;
impl<'a, REG> CDPPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div2)
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div4)
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div8)
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div16)
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div1)
    }
}
///Field `CDPPRE2` reader - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)
pub use CDPPRE1_R as CDPPRE2_R;
///Field `CDPPRE2` writer - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)
pub use CDPPRE1_W as CDPPRE2_W;
impl R {
    ///Bits 4:6 - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)
    #[inline(always)]
    pub fn cdppre1(&self) -> CDPPRE1_R {
        CDPPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)
    #[inline(always)]
    pub fn cdppre2(&self) -> CDPPRE2_R {
        CDPPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDCFGR2")
            .field("cdppre1", &self.cdppre1())
            .field("cdppre2", &self.cdppre2())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)
    #[inline(always)]
    pub fn cdppre1(&mut self) -> CDPPRE1_W<'_, CDCFGR2rs> {
        CDPPRE1_W::new(self, 4)
    }
    ///Bits 8:10 - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)
    #[inline(always)]
    pub fn cdppre2(&mut self) -> CDPPRE2_W<'_, CDCFGR2rs> {
        CDPPRE2_W::new(self, 8)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cdcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#RCC:CDCFGR2)*/
pub struct CDCFGR2rs;
impl crate::RegisterSpec for CDCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cdcfgr2::R`](R) reader structure
impl crate::Readable for CDCFGR2rs {}
///`write(|w| ..)` method takes [`cdcfgr2::W`](W) writer structure
impl crate::Writable for CDCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CDCFGR2 to value 0
impl crate::Resettable for CDCFGR2rs {}
