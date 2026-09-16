///Register `D2CFGR` reader
pub type R = crate::R<D2CFGRrs>;
///Register `D2CFGR` writer
pub type W = crate::W<D2CFGRrs>;
/**D2 domain APB1 prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D2PPRE1 {
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
impl From<D2PPRE1> for u8 {
    #[inline(always)]
    fn from(variant: D2PPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for D2PPRE1 {
    type Ux = u8;
}
impl crate::IsEnum for D2PPRE1 {}
///Field `D2PPRE1` reader - D2 domain APB1 prescaler
pub type D2PPRE1_R = crate::FieldReader<D2PPRE1>;
impl D2PPRE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> D2PPRE1 {
        match self.bits {
            4 => D2PPRE1::Div2,
            5 => D2PPRE1::Div4,
            6 => D2PPRE1::Div8,
            7 => D2PPRE1::Div16,
            _ => D2PPRE1::Div1,
        }
    }
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D2PPRE1::Div2
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D2PPRE1::Div4
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D2PPRE1::Div8
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D2PPRE1::Div16
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), D2PPRE1::Div1)
    }
}
///Field `D2PPRE1` writer - D2 domain APB1 prescaler
pub type D2PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, D2PPRE1, crate::Safe>;
impl<'a, REG> D2PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div2)
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div4)
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div8)
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div16)
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div1)
    }
}
///Field `D2PPRE2` reader - D2 domain APB2 prescaler
pub use D2PPRE1_R as D2PPRE2_R;
///Field `D2PPRE2` writer - D2 domain APB2 prescaler
pub use D2PPRE1_W as D2PPRE2_W;
impl R {
    ///Bits 4:6 - D2 domain APB1 prescaler
    #[inline(always)]
    pub fn d2ppre1(&self) -> D2PPRE1_R {
        D2PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - D2 domain APB2 prescaler
    #[inline(always)]
    pub fn d2ppre2(&self) -> D2PPRE2_R {
        D2PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D2CFGR")
            .field("d2ppre1", &self.d2ppre1())
            .field("d2ppre2", &self.d2ppre2())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - D2 domain APB1 prescaler
    #[inline(always)]
    pub fn d2ppre1(&mut self) -> D2PPRE1_W<'_, D2CFGRrs> {
        D2PPRE1_W::new(self, 4)
    }
    ///Bits 8:10 - D2 domain APB2 prescaler
    #[inline(always)]
    pub fn d2ppre2(&mut self) -> D2PPRE2_W<'_, D2CFGRrs> {
        D2PPRE2_W::new(self, 8)
    }
}
/**RCC Domain 2 Clock Configuration Register

You can [`read`](crate::Reg::read) this register and get [`d2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:D2CFGR)*/
pub struct D2CFGRrs;
impl crate::RegisterSpec for D2CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`d2cfgr::R`](R) reader structure
impl crate::Readable for D2CFGRrs {}
///`write(|w| ..)` method takes [`d2cfgr::W`](W) writer structure
impl crate::Writable for D2CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D2CFGR to value 0
impl crate::Resettable for D2CFGRrs {}
