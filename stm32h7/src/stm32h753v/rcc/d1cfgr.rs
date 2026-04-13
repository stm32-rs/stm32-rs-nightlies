///Register `D1CFGR` reader
pub type R = crate::R<D1CFGRrs>;
///Register `D1CFGR` writer
pub type W = crate::W<D1CFGRrs>;
/**D1 domain AHB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    ///8: sys_ck divided by 2
    Div2 = 8,
    ///9: sys_ck divided by 4
    Div4 = 9,
    ///10: sys_ck divided by 8
    Div8 = 10,
    ///11: sys_ck divided by 16
    Div16 = 11,
    ///12: sys_ck divided by 64
    Div64 = 12,
    ///13: sys_ck divided by 128
    Div128 = 13,
    ///14: sys_ck divided by 256
    Div256 = 14,
    ///15: sys_ck divided by 512
    Div512 = 15,
    ///0: sys_ck not divided
    Div1 = 0,
}
impl From<HPRE> for u8 {
    #[inline(always)]
    fn from(variant: HPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE {
    type Ux = u8;
}
impl crate::IsEnum for HPRE {}
///Field `HPRE` reader - D1 domain AHB prescaler
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPRE {
        match self.bits {
            8 => HPRE::Div2,
            9 => HPRE::Div4,
            10 => HPRE::Div8,
            11 => HPRE::Div16,
            12 => HPRE::Div64,
            13 => HPRE::Div128,
            14 => HPRE::Div256,
            15 => HPRE::Div512,
            _ => HPRE::Div1,
        }
    }
    ///sys_ck divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    ///sys_ck divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    ///sys_ck divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    ///sys_ck divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    ///sys_ck divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    ///sys_ck divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    ///sys_ck divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    ///sys_ck divided by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
    ///sys_ck not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), HPRE::Div1)
    }
}
///Field `HPRE` writer - D1 domain AHB prescaler
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE, crate::Safe>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///sys_ck divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    ///sys_ck divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    ///sys_ck divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    ///sys_ck divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    ///sys_ck divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    ///sys_ck divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    ///sys_ck divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    ///sys_ck divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
    ///sys_ck not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
}
/**D1 domain APB3 prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D1PPRE {
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
impl From<D1PPRE> for u8 {
    #[inline(always)]
    fn from(variant: D1PPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for D1PPRE {
    type Ux = u8;
}
impl crate::IsEnum for D1PPRE {}
///Field `D1PPRE` reader - D1 domain APB3 prescaler
pub type D1PPRE_R = crate::FieldReader<D1PPRE>;
impl D1PPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> D1PPRE {
        match self.bits {
            4 => D1PPRE::Div2,
            5 => D1PPRE::Div4,
            6 => D1PPRE::Div8,
            7 => D1PPRE::Div16,
            _ => D1PPRE::Div1,
        }
    }
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D1PPRE::Div2
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D1PPRE::Div4
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D1PPRE::Div8
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D1PPRE::Div16
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), D1PPRE::Div1)
    }
}
///Field `D1PPRE` writer - D1 domain APB3 prescaler
pub type D1PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, D1PPRE, crate::Safe>;
impl<'a, REG> D1PPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div2)
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div4)
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div8)
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div16)
    }
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div1)
    }
}
///Field `D1CPRE` reader - D1 domain Core prescaler
pub use HPRE_R as D1CPRE_R;
///Field `D1CPRE` writer - D1 domain Core prescaler
pub use HPRE_W as D1CPRE_W;
impl R {
    ///Bits 0:3 - D1 domain AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - D1 domain APB3 prescaler
    #[inline(always)]
    pub fn d1ppre(&self) -> D1PPRE_R {
        D1PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:11 - D1 domain Core prescaler
    #[inline(always)]
    pub fn d1cpre(&self) -> D1CPRE_R {
        D1CPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1CFGR")
            .field("hpre", &self.hpre())
            .field("d1ppre", &self.d1ppre())
            .field("d1cpre", &self.d1cpre())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - D1 domain AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, D1CFGRrs> {
        HPRE_W::new(self, 0)
    }
    ///Bits 4:6 - D1 domain APB3 prescaler
    #[inline(always)]
    pub fn d1ppre(&mut self) -> D1PPRE_W<'_, D1CFGRrs> {
        D1PPRE_W::new(self, 4)
    }
    ///Bits 8:11 - D1 domain Core prescaler
    #[inline(always)]
    pub fn d1cpre(&mut self) -> D1CPRE_W<'_, D1CFGRrs> {
        D1CPRE_W::new(self, 8)
    }
}
/**RCC Domain 1 Clock Configuration Register

You can [`read`](crate::Reg::read) this register and get [`d1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#RCC:D1CFGR)*/
pub struct D1CFGRrs;
impl crate::RegisterSpec for D1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`d1cfgr::R`](R) reader structure
impl crate::Readable for D1CFGRrs {}
///`write(|w| ..)` method takes [`d1cfgr::W`](W) writer structure
impl crate::Writable for D1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1CFGR to value 0
impl crate::Resettable for D1CFGRrs {}
