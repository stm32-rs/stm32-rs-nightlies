///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**Synchronization Overrun Flag %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF0 {
    ///0: No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    NoSyncEvent = 0,
    ///1: Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    SyncEvent = 1,
}
impl From<SOF0> for bool {
    #[inline(always)]
    fn from(variant: SOF0) -> Self {
        variant as u8 != 0
    }
}
///Field `SOF(0-15)` reader - Synchronization Overrun Flag %s
pub type SOF_R = crate::BitReader<SOF0>;
impl SOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOF0 {
        match self.bits {
            false => SOF0::NoSyncEvent,
            true => SOF0::SyncEvent,
        }
    }
    ///No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    #[inline(always)]
    pub fn is_no_sync_event(&self) -> bool {
        *self == SOF0::NoSyncEvent
    }
    ///Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    #[inline(always)]
    pub fn is_sync_event(&self) -> bool {
        *self == SOF0::SyncEvent
    }
}
///Field `SOF(0-15)` writer - Synchronization Overrun Flag %s
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG, SOF0>;
impl<'a, REG> SOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    #[inline(always)]
    pub fn no_sync_event(self) -> &'a mut crate::W<REG> {
        self.variant(SOF0::NoSyncEvent)
    }
    ///Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    #[inline(always)]
    pub fn sync_event(self) -> &'a mut crate::W<REG> {
        self.variant(SOF0::SyncEvent)
    }
}
impl R {
    ///Synchronization Overrun Flag (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SOF0` field.</div>
    #[inline(always)]
    pub fn sof(&self, n: u8) -> SOF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        SOF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Synchronization Overrun Flag (0-15)
    #[inline(always)]
    pub fn sof_iter(&self) -> impl Iterator<Item = SOF_R> + '_ {
        (0..16).map(move |n| SOF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Synchronization Overrun Flag 0
    #[inline(always)]
    pub fn sof0(&self) -> SOF_R {
        SOF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Synchronization Overrun Flag 1
    #[inline(always)]
    pub fn sof1(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization Overrun Flag 2
    #[inline(always)]
    pub fn sof2(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization Overrun Flag 3
    #[inline(always)]
    pub fn sof3(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Synchronization Overrun Flag 4
    #[inline(always)]
    pub fn sof4(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Synchronization Overrun Flag 5
    #[inline(always)]
    pub fn sof5(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Synchronization Overrun Flag 6
    #[inline(always)]
    pub fn sof6(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Synchronization Overrun Flag 7
    #[inline(always)]
    pub fn sof7(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Synchronization Overrun Flag 8
    #[inline(always)]
    pub fn sof8(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Synchronization Overrun Flag 9
    #[inline(always)]
    pub fn sof9(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Synchronization Overrun Flag 10
    #[inline(always)]
    pub fn sof10(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Overrun Flag 11
    #[inline(always)]
    pub fn sof11(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Synchronization Overrun Flag 12
    #[inline(always)]
    pub fn sof12(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Synchronization Overrun Flag 13
    #[inline(always)]
    pub fn sof13(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Synchronization Overrun Flag 14
    #[inline(always)]
    pub fn sof14(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Synchronization Overrun Flag 15
    #[inline(always)]
    pub fn sof15(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("sof0", &self.sof0())
            .field("sof1", &self.sof1())
            .field("sof2", &self.sof2())
            .field("sof3", &self.sof3())
            .field("sof4", &self.sof4())
            .field("sof5", &self.sof5())
            .field("sof6", &self.sof6())
            .field("sof7", &self.sof7())
            .field("sof8", &self.sof8())
            .field("sof9", &self.sof9())
            .field("sof10", &self.sof10())
            .field("sof11", &self.sof11())
            .field("sof12", &self.sof12())
            .field("sof13", &self.sof13())
            .field("sof14", &self.sof14())
            .field("sof15", &self.sof15())
            .finish()
    }
}
impl W {
    ///Synchronization Overrun Flag (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SOF0` field.</div>
    #[inline(always)]
    pub fn sof(&mut self, n: u8) -> SOF_W<'_, CSRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        SOF_W::new(self, n)
    }
    ///Bit 0 - Synchronization Overrun Flag 0
    #[inline(always)]
    pub fn sof0(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 0)
    }
    ///Bit 1 - Synchronization Overrun Flag 1
    #[inline(always)]
    pub fn sof1(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 1)
    }
    ///Bit 2 - Synchronization Overrun Flag 2
    #[inline(always)]
    pub fn sof2(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 2)
    }
    ///Bit 3 - Synchronization Overrun Flag 3
    #[inline(always)]
    pub fn sof3(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 3)
    }
    ///Bit 4 - Synchronization Overrun Flag 4
    #[inline(always)]
    pub fn sof4(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 4)
    }
    ///Bit 5 - Synchronization Overrun Flag 5
    #[inline(always)]
    pub fn sof5(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 5)
    }
    ///Bit 6 - Synchronization Overrun Flag 6
    #[inline(always)]
    pub fn sof6(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 6)
    }
    ///Bit 7 - Synchronization Overrun Flag 7
    #[inline(always)]
    pub fn sof7(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 7)
    }
    ///Bit 8 - Synchronization Overrun Flag 8
    #[inline(always)]
    pub fn sof8(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 8)
    }
    ///Bit 9 - Synchronization Overrun Flag 9
    #[inline(always)]
    pub fn sof9(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 9)
    }
    ///Bit 10 - Synchronization Overrun Flag 10
    #[inline(always)]
    pub fn sof10(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Overrun Flag 11
    #[inline(always)]
    pub fn sof11(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 11)
    }
    ///Bit 12 - Synchronization Overrun Flag 12
    #[inline(always)]
    pub fn sof12(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 12)
    }
    ///Bit 13 - Synchronization Overrun Flag 13
    #[inline(always)]
    pub fn sof13(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 13)
    }
    ///Bit 14 - Synchronization Overrun Flag 14
    #[inline(always)]
    pub fn sof14(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 14)
    }
    ///Bit 15 - Synchronization Overrun Flag 15
    #[inline(always)]
    pub fn sof15(&mut self) -> SOF_W<'_, CSRrs> {
        SOF_W::new(self, 15)
    }
}
/**DMA Multiplexer Channel Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DMAMUX1:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
