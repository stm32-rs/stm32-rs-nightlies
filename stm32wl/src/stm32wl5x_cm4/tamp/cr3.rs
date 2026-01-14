///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
/**ITAMP3NOER

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3NOER {
    ///0: Internal tamper x event erases the backup registers
    Erase = 0,
    ///1: Internal tamper x event does not erase the backup registers
    NotErase = 1,
}
impl From<ITAMP3NOER> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3NOER) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3NOER` reader - ITAMP3NOER
pub type ITAMP3NOER_R = crate::BitReader<ITAMP3NOER>;
impl ITAMP3NOER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3NOER {
        match self.bits {
            false => ITAMP3NOER::Erase,
            true => ITAMP3NOER::NotErase,
        }
    }
    ///Internal tamper x event erases the backup registers
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ITAMP3NOER::Erase
    }
    ///Internal tamper x event does not erase the backup registers
    #[inline(always)]
    pub fn is_not_erase(&self) -> bool {
        *self == ITAMP3NOER::NotErase
    }
}
///Field `ITAMP3NOER` writer - ITAMP3NOER
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3NOER>;
impl<'a, REG> ITAMP3NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal tamper x event erases the backup registers
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3NOER::Erase)
    }
    ///Internal tamper x event does not erase the backup registers
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3NOER::NotErase)
    }
}
///Field `ITAMP5NOER` reader - ITAMP5NOER
pub use ITAMP3NOER_R as ITAMP5NOER_R;
///Field `ITAMP6NOER` reader - ITAMP6NOER
pub use ITAMP3NOER_R as ITAMP6NOER_R;
///Field `ITAMP8NOER` reader - ITAMP8NOER
pub use ITAMP3NOER_R as ITAMP8NOER_R;
///Field `ITAMP5NOER` writer - ITAMP5NOER
pub use ITAMP3NOER_W as ITAMP5NOER_W;
///Field `ITAMP6NOER` writer - ITAMP6NOER
pub use ITAMP3NOER_W as ITAMP6NOER_W;
///Field `ITAMP8NOER` writer - ITAMP8NOER
pub use ITAMP3NOER_W as ITAMP8NOER_W;
impl R {
    ///Bit 2 - ITAMP3NOER
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - ITAMP5NOER
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ITAMP6NOER
    #[inline(always)]
    pub fn itamp6noer(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - ITAMP8NOER
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("itamp3noer", &self.itamp3noer())
            .field("itamp5noer", &self.itamp5noer())
            .field("itamp6noer", &self.itamp6noer())
            .field("itamp8noer", &self.itamp8noer())
            .finish()
    }
}
impl W {
    ///Bit 2 - ITAMP3NOER
    #[inline(always)]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<'_, CR3rs> {
        ITAMP3NOER_W::new(self, 2)
    }
    ///Bit 4 - ITAMP5NOER
    #[inline(always)]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<'_, CR3rs> {
        ITAMP5NOER_W::new(self, 4)
    }
    ///Bit 5 - ITAMP6NOER
    #[inline(always)]
    pub fn itamp6noer(&mut self) -> ITAMP6NOER_W<'_, CR3rs> {
        ITAMP6NOER_W::new(self, 5)
    }
    ///Bit 7 - ITAMP8NOER
    #[inline(always)]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<'_, CR3rs> {
        ITAMP8NOER_W::new(self, 7)
    }
}
/**TAMP control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TAMP:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
