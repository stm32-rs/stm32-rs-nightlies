///Register `HSECR_KEY` writer
pub type W = crate::W<HSECR_KEYrs>;
/**

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum KEY {
    ///3405695742: Write enable key
    Unlock = 3405695742,
}
impl From<KEY> for u32 {
    #[inline(always)]
    fn from(variant: KEY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY {
    type Ux = u32;
}
impl crate::IsEnum for KEY {}
///Field `KEY` writer -
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, KEY>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Write enable key
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Unlock)
    }
}
impl core::fmt::Debug for crate::generic::Reg<HSECR_KEYrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, HSECR_KEYrs> {
        KEY_W::new(self, 0)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsecr_key::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:HSECR_KEY)*/
pub struct HSECR_KEYrs;
impl crate::RegisterSpec for HSECR_KEYrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hsecr_key::W`](W) writer structure
impl crate::Writable for HSECR_KEYrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HSECR_KEY to value 0
impl crate::Resettable for HSECR_KEYrs {}
