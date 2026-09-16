///Register `D3CFGR` reader
pub type R = crate::R<D3CFGRrs>;
///Register `D3CFGR` writer
pub type W = crate::W<D3CFGRrs>;
/**D3 domain APB4 prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D3PPRE {
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
impl From<D3PPRE> for u8 {
    #[inline(always)]
    fn from(variant: D3PPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for D3PPRE {
    type Ux = u8;
}
impl crate::IsEnum for D3PPRE {}
///Field `D3PPRE` reader - D3 domain APB4 prescaler
pub type D3PPRE_R = crate::FieldReader<D3PPRE>;
impl D3PPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> D3PPRE {
        match self.bits {
            4 => D3PPRE::Div2,
            5 => D3PPRE::Div4,
            6 => D3PPRE::Div8,
            7 => D3PPRE::Div16,
            _ => D3PPRE::Div1,
        }
    }
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D3PPRE::Div2
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D3PPRE::Div4
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D3PPRE::Div8
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D3PPRE::Div16
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), D3PPRE::Div1)
    }
}
///Field `D3PPRE` writer - D3 domain APB4 prescaler
pub type D3PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, D3PPRE, crate::Safe>;
impl<'a, REG> D3PPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div2)
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div4)
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div8)
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div16)
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div1)
    }
}
impl R {
    ///Bits 4:6 - D3 domain APB4 prescaler
    #[inline(always)]
    pub fn d3ppre(&self) -> D3PPRE_R {
        D3PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3CFGR")
            .field("d3ppre", &self.d3ppre())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - D3 domain APB4 prescaler
    #[inline(always)]
    pub fn d3ppre(&mut self) -> D3PPRE_W<'_, D3CFGRrs> {
        D3PPRE_W::new(self, 4)
    }
}
/**RCC Domain 3 Clock Configuration Register

You can [`read`](crate::Reg::read) this register and get [`d3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:D3CFGR)*/
pub struct D3CFGRrs;
impl crate::RegisterSpec for D3CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`d3cfgr::R`](R) reader structure
impl crate::Readable for D3CFGRrs {}
///`write(|w| ..)` method takes [`d3cfgr::W`](W) writer structure
impl crate::Writable for D3CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3CFGR to value 0
impl crate::Resettable for D3CFGRrs {}
