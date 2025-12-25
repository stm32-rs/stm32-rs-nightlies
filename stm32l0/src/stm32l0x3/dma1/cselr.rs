///Register `CSELR` reader
pub type R = crate::R<CSELRrs>;
///Register `CSELR` writer
pub type W = crate::W<CSELRrs>;
/**DMA channel 1 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1S {
    ///0: Default mapping
    NoMapping = 0,
    ///1: Mapping 1
    Map1 = 1,
    ///2: Mapping 2
    Map2 = 2,
    ///3: Mapping 3
    Map3 = 3,
    ///4: Mapping 4
    Map4 = 4,
    ///5: Mapping 5
    Map5 = 5,
    ///6: Mapping 6
    Map6 = 6,
    ///7: Mapping 7
    Map7 = 7,
    ///8: Mapping 8
    Map8 = 8,
    ///9: Mapping 9
    Map9 = 9,
    ///10: Mapping 10
    Map10 = 10,
    ///11: Mapping 11
    Map11 = 11,
    ///12: Mapping 12
    Map12 = 12,
    ///13: Mapping 13
    Map13 = 13,
    ///14: Mapping 14
    Map14 = 14,
    ///15: Mapping 15
    Map15 = 15,
}
impl From<C1S> for u8 {
    #[inline(always)]
    fn from(variant: C1S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1S {
    type Ux = u8;
}
impl crate::IsEnum for C1S {}
///Field `C1S` reader - DMA channel 1 selection
pub type C1S_R = crate::FieldReader<C1S>;
impl C1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1S {
        match self.bits {
            0 => C1S::NoMapping,
            1 => C1S::Map1,
            2 => C1S::Map2,
            3 => C1S::Map3,
            4 => C1S::Map4,
            5 => C1S::Map5,
            6 => C1S::Map6,
            7 => C1S::Map7,
            8 => C1S::Map8,
            9 => C1S::Map9,
            10 => C1S::Map10,
            11 => C1S::Map11,
            12 => C1S::Map12,
            13 => C1S::Map13,
            14 => C1S::Map14,
            15 => C1S::Map15,
            _ => unreachable!(),
        }
    }
    ///Default mapping
    #[inline(always)]
    pub fn is_no_mapping(&self) -> bool {
        *self == C1S::NoMapping
    }
    ///Mapping 1
    #[inline(always)]
    pub fn is_map1(&self) -> bool {
        *self == C1S::Map1
    }
    ///Mapping 2
    #[inline(always)]
    pub fn is_map2(&self) -> bool {
        *self == C1S::Map2
    }
    ///Mapping 3
    #[inline(always)]
    pub fn is_map3(&self) -> bool {
        *self == C1S::Map3
    }
    ///Mapping 4
    #[inline(always)]
    pub fn is_map4(&self) -> bool {
        *self == C1S::Map4
    }
    ///Mapping 5
    #[inline(always)]
    pub fn is_map5(&self) -> bool {
        *self == C1S::Map5
    }
    ///Mapping 6
    #[inline(always)]
    pub fn is_map6(&self) -> bool {
        *self == C1S::Map6
    }
    ///Mapping 7
    #[inline(always)]
    pub fn is_map7(&self) -> bool {
        *self == C1S::Map7
    }
    ///Mapping 8
    #[inline(always)]
    pub fn is_map8(&self) -> bool {
        *self == C1S::Map8
    }
    ///Mapping 9
    #[inline(always)]
    pub fn is_map9(&self) -> bool {
        *self == C1S::Map9
    }
    ///Mapping 10
    #[inline(always)]
    pub fn is_map10(&self) -> bool {
        *self == C1S::Map10
    }
    ///Mapping 11
    #[inline(always)]
    pub fn is_map11(&self) -> bool {
        *self == C1S::Map11
    }
    ///Mapping 12
    #[inline(always)]
    pub fn is_map12(&self) -> bool {
        *self == C1S::Map12
    }
    ///Mapping 13
    #[inline(always)]
    pub fn is_map13(&self) -> bool {
        *self == C1S::Map13
    }
    ///Mapping 14
    #[inline(always)]
    pub fn is_map14(&self) -> bool {
        *self == C1S::Map14
    }
    ///Mapping 15
    #[inline(always)]
    pub fn is_map15(&self) -> bool {
        *self == C1S::Map15
    }
}
///Field `C1S` writer - DMA channel 1 selection
pub type C1S_W<'a, REG> = crate::FieldWriter<'a, REG, 4, C1S, crate::Safe>;
impl<'a, REG> C1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Default mapping
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::NoMapping)
    }
    ///Mapping 1
    #[inline(always)]
    pub fn map1(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map1)
    }
    ///Mapping 2
    #[inline(always)]
    pub fn map2(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map2)
    }
    ///Mapping 3
    #[inline(always)]
    pub fn map3(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map3)
    }
    ///Mapping 4
    #[inline(always)]
    pub fn map4(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map4)
    }
    ///Mapping 5
    #[inline(always)]
    pub fn map5(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map5)
    }
    ///Mapping 6
    #[inline(always)]
    pub fn map6(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map6)
    }
    ///Mapping 7
    #[inline(always)]
    pub fn map7(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map7)
    }
    ///Mapping 8
    #[inline(always)]
    pub fn map8(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map8)
    }
    ///Mapping 9
    #[inline(always)]
    pub fn map9(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map9)
    }
    ///Mapping 10
    #[inline(always)]
    pub fn map10(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map10)
    }
    ///Mapping 11
    #[inline(always)]
    pub fn map11(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map11)
    }
    ///Mapping 12
    #[inline(always)]
    pub fn map12(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map12)
    }
    ///Mapping 13
    #[inline(always)]
    pub fn map13(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map13)
    }
    ///Mapping 14
    #[inline(always)]
    pub fn map14(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map14)
    }
    ///Mapping 15
    #[inline(always)]
    pub fn map15(self) -> &'a mut crate::W<REG> {
        self.variant(C1S::Map15)
    }
}
///Field `C2S` reader - DMA channel 2 selection
pub use C1S_R as C2S_R;
///Field `C3S` reader - DMA channel 3 selection
pub use C1S_R as C3S_R;
///Field `C4S` reader - DMA channel 4 selection
pub use C1S_R as C4S_R;
///Field `C5S` reader - DMA channel 5 selection
pub use C1S_R as C5S_R;
///Field `C6S` reader - DMA channel 6 selection
pub use C1S_R as C6S_R;
///Field `C7S` reader - DMA channel 7 selection
pub use C1S_R as C7S_R;
///Field `C2S` writer - DMA channel 2 selection
pub use C1S_W as C2S_W;
///Field `C3S` writer - DMA channel 3 selection
pub use C1S_W as C3S_W;
///Field `C4S` writer - DMA channel 4 selection
pub use C1S_W as C4S_W;
///Field `C5S` writer - DMA channel 5 selection
pub use C1S_W as C5S_W;
///Field `C6S` writer - DMA channel 6 selection
pub use C1S_W as C6S_W;
///Field `C7S` writer - DMA channel 7 selection
pub use C1S_W as C7S_W;
impl R {
    ///Bits 0:3 - DMA channel 1 selection
    #[inline(always)]
    pub fn c1s(&self) -> C1S_R {
        C1S_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - DMA channel 2 selection
    #[inline(always)]
    pub fn c2s(&self) -> C2S_R {
        C2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - DMA channel 3 selection
    #[inline(always)]
    pub fn c3s(&self) -> C3S_R {
        C3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DMA channel 4 selection
    #[inline(always)]
    pub fn c4s(&self) -> C4S_R {
        C4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - DMA channel 5 selection
    #[inline(always)]
    pub fn c5s(&self) -> C5S_R {
        C5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - DMA channel 6 selection
    #[inline(always)]
    pub fn c6s(&self) -> C6S_R {
        C6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - DMA channel 7 selection
    #[inline(always)]
    pub fn c7s(&self) -> C7S_R {
        C7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSELR")
            .field("c1s", &self.c1s())
            .field("c7s", &self.c7s())
            .field("c6s", &self.c6s())
            .field("c5s", &self.c5s())
            .field("c4s", &self.c4s())
            .field("c3s", &self.c3s())
            .field("c2s", &self.c2s())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DMA channel 1 selection
    #[inline(always)]
    pub fn c1s(&mut self) -> C1S_W<'_, CSELRrs> {
        C1S_W::new(self, 0)
    }
    ///Bits 4:7 - DMA channel 2 selection
    #[inline(always)]
    pub fn c2s(&mut self) -> C2S_W<'_, CSELRrs> {
        C2S_W::new(self, 4)
    }
    ///Bits 8:11 - DMA channel 3 selection
    #[inline(always)]
    pub fn c3s(&mut self) -> C3S_W<'_, CSELRrs> {
        C3S_W::new(self, 8)
    }
    ///Bits 12:15 - DMA channel 4 selection
    #[inline(always)]
    pub fn c4s(&mut self) -> C4S_W<'_, CSELRrs> {
        C4S_W::new(self, 12)
    }
    ///Bits 16:19 - DMA channel 5 selection
    #[inline(always)]
    pub fn c5s(&mut self) -> C5S_W<'_, CSELRrs> {
        C5S_W::new(self, 16)
    }
    ///Bits 20:23 - DMA channel 6 selection
    #[inline(always)]
    pub fn c6s(&mut self) -> C6S_W<'_, CSELRrs> {
        C6S_W::new(self, 20)
    }
    ///Bits 24:27 - DMA channel 7 selection
    #[inline(always)]
    pub fn c7s(&mut self) -> C7S_W<'_, CSELRrs> {
        C7S_W::new(self, 24)
    }
}
/**channel selection register

You can [`read`](crate::Reg::read) this register and get [`cselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#DMA1:CSELR)*/
pub struct CSELRrs;
impl crate::RegisterSpec for CSELRrs {
    type Ux = u32;
}
///`read()` method returns [`cselr::R`](R) reader structure
impl crate::Readable for CSELRrs {}
///`write(|w| ..)` method takes [`cselr::W`](W) writer structure
impl crate::Writable for CSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSELR to value 0
impl crate::Resettable for CSELRrs {}
